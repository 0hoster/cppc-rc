use std::{env,fs};
use std::path::PathBuf;
use text_colorizer::*;

const MAX_SOURCE: u64 = 10000;
const SKIP_DIRS: [&str; 1]= [".git",];
const EXECUTE_EXTENSION: [&str; 3] = ["exe","o","out",];

fn main() ->Result<(),std::io::Error>{
    let arg = env::args().skip(1).next();
    let targer_dir:PathBuf;
    if let Some(dir) = arg{
       targer_dir = dir.into();
    }else{
       targer_dir = env::current_dir()?;
    }
    println!("State Size    Path");
    let mut base_string;
    let base_dir = if let Some(base) = targer_dir.to_str(){
        base_string = base.to_string();
        base_string.push('/');
        base_string.as_str()
    }else{
        ""
    };
    let _ = delete_garbage(&targer_dir,base_dir);
    Ok(())
}

fn delete_garbage(path:&PathBuf,base:&str)->Result<(),std::io::Error>{
    for entry in fs::read_dir(path)?.into_iter().filter_map(|e| e.ok()){
        if let Ok(metadata) = entry.metadata(){
            if metadata.is_dir(){
                let should_skip = SKIP_DIRS.into_iter().any(|x|x==entry.file_name());
                if should_skip{
                    continue;
                }
                let _ = delete_garbage(&entry.path(),base);
            }else{
                let execute = if let Some(extension) = entry.path().extension(){
                    EXECUTE_EXTENSION.into_iter().any(|x|x==extension)
                }else{
                    true
                };
                if (metadata.len()>MAX_SOURCE && execute)||metadata.len()==0{
                    println!("{}",format!("{} {:<8}{}","Del  ".red(),metadata.len(),&entry.path().to_str().unwrap_or("Unknown").replace(base,"").bold()).bright_yellow());
                    let _ = fs::remove_file(entry.path());
                }else if metadata.len()>MAX_SOURCE{
                    println!("{}",format!("{} {:<8}{}","Large".cyan(),metadata.len(),&entry.path().to_str().unwrap_or("Unknown").replace(base,"")));
                }else{
                    println!("{} {:<8}{}","Ok   ".green(),metadata.len(),&entry.path().to_str().unwrap_or("Unknown").replace(base,"").dimmed());
                }
            }
        }
    }
    Ok(())
}
