use clap::{command,Arg};
use dirs::home_dir;
use std::fs;

struct MSPaths{
    word: String,
    excel: String,
    ppt: String
}

fn main(){
    let matches = command!("rm_recents_cli")
        .version("1.0")
        .author("kenf1")
        .about("CLI app to remove recents list for ≥ 1 of following: MS Word, Excel, or PowerPoint.")
        .arg(
            Arg::new("app_name")
                .short('n')
                .long("app_name")
                .value_name("APPNAME")
                .help("App name: word, excel, ppt, or all")
                .required(true)
        )
        .get_matches();

    if matches.args_present(){
        //convert arg into string
        let input = matches.get_one::<String>("app_name").unwrap().to_string();
        
        //rm files based on user entry
        let home_path = getHomePath();
        rmFile(input,home_path);
        println!("Success!");
    }
}

//return HOME path if found, else return NA
fn getHomePath() -> String{
    if let Some(path) = home_dir(){
        let home_path = path.to_string_lossy().into_owned();
        return home_path;
    }else{
        return String::from("NA");
    }
}

//rm files based on user entry
fn rmFile(input: String,home_path: String){
    //create full_path from HOME + plist path
    let full_path = MSPaths{
        word: home_path.clone()+"/Library/Containers/com.microsoft.Word/Data/Library/Preferences/com.microsoft.Word.securebookmarks.plist",
        excel: home_path.clone()+"/Library/Containers/com.microsoft.Excel/Data/Library/Preferences/com.microsoft.Excel.securebookmarks.plist",
        ppt: home_path.clone()+"/Library/Containers/com.microsoft.Powerpoint/Data/Library/Preferences/com.microsoft.Powerpoint.securebookmarks.plist"
    };

    let _ = match input.trim(){
        "word" => fs::remove_file(full_path.word),
        "excel" => fs::remove_file(full_path.excel),
        "ppt" => fs::remove_file(full_path.ppt),
        "all" => Ok({
            let _ = fs::remove_file(full_path.word);
            let _ = fs::remove_file(full_path.excel);
            let _ = fs::remove_file(full_path.ppt);
        }),
        _ => Ok({
            println!("Please try again with one of the accepted values.\n Use --help to see all accepted values.");
        })
    };
}