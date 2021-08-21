import json
import numpy as np

def GetVector(cube,scenario,dt_idx_start,dt_idx_end,start,end):
    for d in range(dt_idx_start,dt_idx_end):
        startIdx=scenario*len(cube['dates'])*cube['num_series']+d*cube['num_series']+start
        endIdx=scenario*len(cube['dates'])*cube['num_series']+d*cube['num_series']+end
        vector=cube['data'][startIdx:endIdx]
        if d==dt_idx_start:
            retVector=np.asarray(vector)
        else:
            retVector=np.vstack((retVector,vector))
        #print(str(startIdx)+' - '+str(endIdx))
    return retVector

def LoadCube(path,verbose=False):
    f = open(path,)
    cube=json.load(f)
    #num_terms
    if verbose:
        print('Number of series: '+str(cube['num_series']))
        print('Number of scenarios: '+str(cube['num_scenarios']))
        print('Dates ('+str(len(cube['dates']))+'): '+str(cube['dates']))
    return cube

def GetIndex(cube,d,s,ts_idx):
    return s*len(cube['dates'])*cube['num_series']+d*cube['num_series']+ts_idx

def GetValue(cube,d,s,ts_idx):
    return cube['data'][s*len(cube['dates'])*cube['num_series']+d*cube['num_series']+ts_idx]

def GetScenarioData(cube,s):
    data=np.asarray(cube['data'][s*len(cube['dates'])*cube['num_series']:(s+1)*len(cube['dates'])*cube['num_series']])
    data=data.reshape((len(cube['dates']),cube['num_series']))
    return data

def LoadModelValuesCubeAsTable(path,verbose=False,scenario_start=-1,scenario_end=-1):
    f = open(path,)
    cube=json.load(f)
    #num_terms
    if verbose:
        print('Number of series: '+str(cube['num_series']))
        print('Number of scenarios: '+str(cube['num_scenarios']))
        print('Dates ('+str(len(cube['dates']))+'): '+str(cube['dates']))
    
    table=[]
    for ts in range(0,cube['num_series']):
        ts_name=cube['time_series_names'][ts]
        #print(ts_name)
        idx=ts_name.index('[')
        term=ts_name[idx+1:len(ts_name)-1]
        model=ts_name[0:idx-2]
        for dt in range(0,len(cube['dates'])):
            #print(str(dt)+': '+str(cube['dates'][dt]))
            for s in range(0,cube['num_scenarios']):
                if (scenario_start==-1 or s>=scenario_start) and (scenario_end==-1 or s<=scenario_end):
                    value=GetValue(cube,dt,s,ts)
                    line=[s,ts,dt,cube['time_series_names'][ts],cube['dates'][dt],model,float(term),value]
                    table.append(line)
    return table