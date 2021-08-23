# Monaco

Monaco is a Rust application for Monte Carlo (financial) exposure simulation.

The 'monaco' executable relies on logic defined in the 'monaco-lib' library.

## Mode of execution

The application is invoked with 'monaco *input_dir*' (where *input_dir* is the folder containing all the input files).

The execution is completely controlled via the contents of the input files.

All the results are stored in a set of output files.

## Dependencies

'Monaco' depends on the follwing libraries:

- 'chrono' (for log timestamps)
- 'serde' (for json deserialization/serialization);
- 'rand' (for radom number generation)

> The dependency on 'chrono' is needed only for the adding the date and time to log entries. It could be easily removed if needed.

## Input files

The application expects at least 4 input files: 
- a 'control.json' file (that contains the execution hyper-parameters)
- at least one model definition file
- any number of instrument definition files
- a 'Correlations.json' file (that contains the corelations between the models' variables).
---
### Control.json

This file is used to control execution. It contains a simple json object. These parameters have to be set:
|Paramter|Description|Example|
|---|---|---|
|log_tags|List of log entry types to show|["app","controller"]|
|num_paths|Number of simulation scenarios (paths)|10000|
|time_steps|Vector of dates at which to simulate factors and comput exposures|\[0.25,0.5,0.75,1\]|
|output_file_variables|Name of output file for generated random variates|"C:/MyFolder/variables_cube.json"|
|output_file_outputs|Name of output file for generated model outputs|"C:/MyFolder/outputs_cube.json"|
|output_file_exposures|Name of output file for instrument exposures|"C:/MyFolder/exposures_cube.json"|
|dump_models|Specifies whether to write actual model objects|true|
|model_output_dir|If 'dump_models' is true: folder where to store model values|"C:/MyFolder/output"|
|dump_model_values|Specifies whether to output also diagnostic rate model term structures|true|
|model_values_terms|if 'dump_model_values' is true: term structure gridpoint for which calculate diagnostic values|\[0.25,0.5,1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0,20.0,25.0,30.0\]|,
|output_file_model_values"|If 'dump_model_values' is true: name of the file containing term structure values|"C:/MyFolder/model_values_cube.json"|

> The list of available log tags is: app, controller, lsm, instrument, model

### Correlations.json

This file contains a json list of the correlation values between the model random variables.
The list is in `row,column` format and the values should be order alphabetically using the models' names and each model variable ordering convention.

### Model definitions

The application will need at least one model defined to generate scenarios.
The model files are named `model_ModelName` where `model` is the model type and `ModelName` is the specific model implementation (e.g. `hw1f_USD.json`).

See the 'Models' section for model specific parameters.

### Instrument definitions

The instruments to be evaluated must be passed in a series of json files (one for each position). The files must be placed in the config folder along with all other input files.
The instrument files are named `instrument_PositionName` where `instrument` is the instrument type and `PositionName` is the name of the position (e.g. `vanilla-swap_MySwap`).

See the 'Instruments' section for the available instruments and their specific definitions.

> Although the main purpose of this application is to price financial isntruments over the simulated scenarios, it is not mandatory to have any instrument defined.

## Output files

The application generates the follwing files:

|File name|Decsription|Format|
|---|---|---|
|`control.output_file_variables`|Model variates|Data cube in json format \[number of simulation dates,number of scenarios,number of model variables\]|
|`control.output_file_outputs`|Model outputs|Data cube in json format \[number of simulation dates,number of scenarios,number of model outputs\]|
|`control.output_file_exposures`|Exposure values|Data cube in json format \[number of simulation dates,number of scenarios,number of positions\]|
|`control.output_file_model_values`|Model explicit values (when `control.dump_model_values` is set to 'true'|Data cube in json format [number of simulation dates,number of scenarios,number of model outputs*length of `control.model_values_terms`]|
|*ModelName*|Initialized model|json|

> The live models are dumped to capture the results of the 'init' method (e.g. hw1f thetas).

# Behaviour

The 'monaco' application reads the input files in the folder specified as its argument and performs the following:

1. generate the (correlated) random variates necessary to the models;
2. ask the models to create their outputs given the generated variates;
3. price every instrument for each scenario and at each time step in the model ouputs data cube
4. Save the exposure cube generated in the previous step (and all the required intermediate results)

> The basic assumption behind the model simulation is that each model takes *n* inputs and has *m* outputs. If *N* is the sum of the number of inputs across all models and *M* is the sum of their number of outputs, then the output of step 1 is a datacube with N time series (one for each simulated random variable) and the output of step 2 is a data cube with M time series.

> For instance, the Hull-White one-factor model and the Black model are (n:1,m:1) models. The Hull-White two factor model, on the other hand, would be a (n:2,m:1) model.

# Data cube

The most important data structure in 'monaco' is the 'data cube' (struct 'Cube' in the module 'data-cube').
It represent a cube of double precision floating point values along three dimensions:

- scenarios
- dates
- time series

It can be visualized a series of *S* matrices with *D* rows and *TS* columns, where *S* is the number of scenarios, *D* is the number of dates, and *TS* the number of time series.

The *D* simulation dates are stored in the 'dates' attribute of the cube.
The *TS* time series names are stored in the 'time_series_names' attribute of the cube.

The dates, in this context and as everywhere else in 'monaco' are date fractions, with 0 being the evaluation date.
The dates are in *ascending* order.

> This means that the value for date 0, time series 2, and scenario 5 corresponds to the third value of the first row of the sixth scenario matrix.

This data structure is, for instance, used to store simulated random variates, model outputs and simulated exposures.

# Conventions

Here are the main conventions used in the application:

- dates are expressed as year fractions (the chrono dependency is used only to decorate log entries)
- the date of the analysis is t=0
- the value of an instrument is zero on the maturity date. More in general, cashflows at time *t* are **not** part of the instrument's value at time *t*

# Models

## Hull-White one factor

The Hull-White one factor ('hw1f_*') model parameters are:

|Parameter|Type|Description|Example|
|---|---|---|---|
|name|String|Model name|"ir_usd"| 
|term_structure|List of \[term,rate\] items|Initial term structure to which the model is fitted|\[\[0.5,0.02\],\[1.0,0.021\],\[2.0,0.019\]\]|
|thetas|List of \[term,value\] items|Theta parameter vector|\[\]|
|a||List of \[term,value\] items|Mean reversion parameter over time|\[\[0.0,0.05\],\[1.0,0.05\]\]|
|sigmas|List of \[term,value\] items|Short rate volatility over time|
|initial_rate|Number|Initial value for the short rate|0.01|\[\[0.0,0.001\],\[1.0,0.0015],\[2.0,0.002\]\]|

> The 'thetas' parameter is usually left empty, as the values are generated by the 'init' function.

## Black

The Black ('black_*') model parameters are:

|Parameter|Type|Description|Example|
|---|---|---|---|
|name|String|Model name|"fx_eur"|
|r|Number|risk-free rate|0.0007807|
|sigmas|List of \[term,value\] items|Value volatility over time|\[\[0.5,0.0007\],\[1.0,0.0008]\]\]|
initial_value|Number|Initial model value|1.190521668|

## Fixed

The fixed ('fixed_*') model is a special model used to provide a constant value with no evolution. It is normally used to provide the FX rate value for the base currency (where it always returns 1).
It features no variables and no outputs, therefore no time series associated to it appears in the variables and output cubes.

|Parameter|Type|Description|Example|
|---|---|---|---|
|name|String|Model name|"fx_usd"|
|value|Number|Fixed value|1.0|

# The LSM algorithm

The 'instrument' module houses all the different financial instrument specifications. New instrument type are added here.
The 'instrument' trait requires the implementation of only two methods:

- 'get_name', which returns the name of the concrete isntrument position (usually just echoing a backing field)
- 'compute_values' which populates the appropriate section of the exposures data cube and returns a cashflows structure and a binary data cube containing exercise events

There are, by design, no constraints as to how to perform the calculation in 'compute_values'. However, the 'lsm' module has been implemented to simplify and standardise the calculations for derivatives using the Longstaff-Schwartz Monte Carlo method.

The 'compute_lsm_values' function requires the following arguments:

|Signature|Description|
|---|---|
|instrument_values_cube:&mut Cube|Data cube with one series, the number of scenarios specified and the required evaluation dates|
|live_models:&HashMap&lt;String,LiveModel&gt;|The universe of live models|
|exercise_flags:&Vec&lt;bool&gt;|A vector with same dimension as the dates in the instrument_values_cube the indicates whether exercise is possible on that date|
|f_models_variables_values:&mut impl FnMut(f64,&HashMap&lt;String,LiveModel&gt;) -> Vec&lt;f64&gt;|Function that returns model variable values|
|f_exercise_value:&mut impl FnMut(usize,f64,&HashMap&lt;String,LiveModel&gt;) -> f64|A function used to compute the exercise value of the instrument|
|f_cashflows:&mut impl FnMut(usize,f64,f64,&HashMap&lt;String,LiveModel&gt;) -> Vec&lt;(f64,f64)&gt;|Function that computes the cashflows along a path between two dates|
|discount_model:&LiveModel|Model to use to move cashflow and exposure values in time|
|logger:&Logger|Logger object (normally the one passed to 'compute_values')|

It populates the instrument values cube and outputs a list cashflow vectors and an exercise cube. 

> The list of dates for which to perform the calculation is the union of the simulation dates, the instrument payment dates, the exercise dates, and the maturity date.

The algorithm for the lsm function performs a backward calculation on the vector of dates that is passed in the 'intrument_values_cube'.
The value of the instrument is zero at maturity.
For every earlier date:

- if it is not an exercise date the value is given by the regression over all previous cashflows for all scenarios
- if it is an exercise date, the instrument value is given by the bigger of the exercise value (given by 'f_exercise_value') and the regression value *calculated over the in-the-money paths*. 

> The cashflows structure returned by 'compute_values' and 'compute_lsm_values' (Vec&lt;Vec&lt;(f64,f64)&gt;&gt;) is a vector of dimension S (number of scenarios) whose entries contain the ordered cashflows for that scenario. Every cashflow is a (t,v) tuple where t is the time of the cashflow (expresed as a year fraction) and v is the value of the cashflow *at time t* (i.e. the cashflows are not discounted).

# Instruments

## Vanilla swap

The vanilla swap instrument can be used to define plain vanilla IRS, basis swaps, and xCCY swaps.
It is defined using these parameters:

|Parameter|Type|Description|Example|
|---|---|---|---|
|name|String|Model name|"MyVanillaSwap"|
|legs|List of leg definitions|One or more swap legs (not necessarily two)|See definition below|

Every leg is defined with these parameters:

|Parameter|Type|Description|Example|
|---|---|---|---|
|notional|Number|Notional value of the swap|1000000|
|pay_or_receive|String|Can be 'pay' or 'receive'|'pay'| 
|discount_model_name|String|Model used for discounting|"ir_usd"|
|projection_model_name|String|Model used for projecting cashflows|"ir_usd"|
|fx_model_name|String|Model used for translating cashflow values into the base ccy|"fx_eur"|
|payment_dates|List of numbers|Dates at which payments are made|\[0.5,1.0,1.5,2.0\]|
|is_fixed|Boolean|Specifies whether the leg is fixed or floating|true|
|fixed_values|List of numbers|List of fixed payments|\[0.005,0.005,0.005,0.005\]|

> When a leg is floating, the first coupon is always fixed. This means that a forward-starting swap can be defined with legs having a zero fixed payment on the start date.

> The 'fixed_values' field is used for fixed coupons but also to add spreads to floating legs.

> For simplicity the reset dates are the previous payment dates.

## Callable swap

A callable swap is defined using these parameters:

|Parameter|Type|Description|Example|
|---|---|---|---|
|name|String|Model name|"MyCallableSwap"|
|exposure_discount_model_name|String|Name of the model to use to discount exposure in the lsm algorithm|"hw1f"|
|call_dates|List of dates|Dates at which the model can be called (cancelled)|\[0.5,1.0,1.5,2.0\]|
|underlying|Vanilla swap definition|Definition of the underlying swap|See 'Vanilla swap' section|