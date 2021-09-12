use chrono;

pub struct Logger
{
    pub log_tags:Vec<(String,usize)>
}

impl Logger
{
    pub fn log<S: Into<String>>(&self,log_msg:S,tag:&str) -> ()
    {
        let lm: String =log_msg.into();
        if self.log_tags.iter().any(|t| t.0.to_lowercase()==tag.to_lowercase()) 
        {   
            println!("[{}] {}",chrono::offset::Local::now(),lm);
        }
    }
    pub fn log_with_check<S: Into<String>>(&self,log_msg:S,tag:&str,verbosity:usize) -> ()
    {
        let lm: String =log_msg.into();
        if self.log_tags.iter().any(|t| t.0.to_lowercase()==tag.to_lowercase() && verbosity<=t.1) 
        {   
            println!("[{}] {}",chrono::offset::Local::now(),lm);
        }
    }
}