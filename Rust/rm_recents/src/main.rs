use std::io;
use dirs::home_dir;
use std::fs;

//store all full paths
struct MSPaths{
    word: String,
    excel: String,
    ppt: String
}

fn main(){
    let mut input: String = String::new();
    let num_args: u32 = 5;
    let home_path: String = getHomePath();
    
    println!("This application will clear the recent files list for 1 or more MS Office applications on macOS. \n Script by KF: https://github.com/kenf1 \n");
    println!("HOME path is located at: {}\n",getHomePath());

    //run if HOME path found
    if home_path != "NA"{
        rmFile(input,num_args,home_path);
    }else{
        println!("Home path not found.");
    }
}

//obtain home path as string
fn getHomePath() -> String{
    //return HOME path if found, else return NA
    if let Some(path) = home_dir(){
        let home_path = path.to_string_lossy().into_owned();
        return home_path;
    }else{
        return String::from("NA");
    }
}

//prompt user + rm files based on user choice
fn rmFile(mut input: String,num_args: u32,home_path: String){
    let full_path = MSPaths{
        word: home_path.clone()+"/Library/Containers/com.microsoft.Word/Data/Library/Preferences/com.microsoft.Word.securebookmarks.plist",
        excel: home_path.clone()+"/Library/Containers/com.microsoft.Excel/Data/Library/Preferences/com.microsoft.Excel.securebookmarks.plist",
        ppt: home_path.clone()+"/Library/Containers/com.microsoft.Powerpoint/Data/Library/Preferences/com.microsoft.Powerpoint.securebookmarks.plist"
    };

    //prompt user
    loop{
        println!("Enter number of choice [1 to {}]:\n 1. Word\n 2. Excel\n 3. PowerPoint\n 4. All\n 5. Exit",num_args);
        
        //user input
        io::stdin().read_line(&mut input).expect("Failed to read line.");
        
        let num = match input.trim().parse::<u32>(){
            Ok(num) => num,
            Err(_) => { //anything not u32
                println!("Invalid input, please try again");
                continue;
            }
        };

        //rm file based on user choice
        match num{
            1 => fs::remove_file(full_path.word),
            2 => fs::remove_file(full_path.excel),
            3 => fs::remove_file(full_path.ppt),
            4 => Ok({ //let _ used to ignore result
                let _ = fs::remove_file(full_path.word);
                let _ = fs::remove_file(full_path.excel);
                let _ = fs::remove_file(full_path.ppt);
            }),
            5 => {
                println!("Exited");
                break;
            },
            _ => {
                println!("Accepted values are: 1 to {}. Please try again.",num_args);
                break;
            }
        };
        println!("Success!");
        break; //exit on success
    }
}