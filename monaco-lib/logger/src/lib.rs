use chrono;

pub struct Logger
{
    pub log_tags:Vec<String>
}

impl Logger
{
    pub fn log<S: Into<String>>(&self,log_msg:S,tag:&str) -> ()
    {
        let lm: String =log_msg.into();
        if self.log_tags.iter().any(|t| t.to_lowercase()==tag.to_lowercase()) 
        {   
            println!("[{}] {}",chrono::offset::Local::now(),lm);
        }
    }
}