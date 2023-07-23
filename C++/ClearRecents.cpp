/*
    clear MS recents list for macOS versions of Word, Excel, PowerPoint
*/

#include <iostream>
#include <cstdio>

int promptUser();
void rmFile(std::string file);
void glueFunc(int choice);
void printPath();

int main(){
    // printPath();
    // std::cout << "\n";
    glueFunc(promptUser());
    return 0;
}

//prompt user for choice
int promptUser(){
    std::cout << "This application will clear the recent files list for 1 or more MS Office applications on macOS. \n Script by KF: https://github.com/kenf1 \n";

    int choice;

    //loop until accepted input entered
    do{
        std::cout << "\n Enter the number of the application's recent list you would like to clear? \n  1. Word \n  2. Excel \n  3. PowerPoint \n  4. All \n  5. Exit \n";
        std::cin >> choice;

        //throw error if non-int entered
        if(std::cin.fail()){
            std::cout << "Only integers are accepted.\n";
            std::cin.clear();
            std::cin.ignore(10000,'\n');
        }
        choice--;
    }while(choice<0 || choice>4);

    return choice;
}

//remove file(s)
void rmFile(std::string file){
    //concat home path to plist path -> full path
    std::string fullPath = std::getenv("HOME")+file;

    //output message for success/fail
    if(std::remove(fullPath.c_str())!=0){
        std::perror("Error deleting file");
    }else{
        std::puts("File successfully deleted");
    }
}

//combine all subfunctions together
void glueFunc(int choice){
    //full path to plist files
    const std::string paths[] = {"/Library/Containers/com.microsoft.Word/Data/Library/Preferences/com.microsoft.Word.securebookmarks.plist",
        "/Library/Containers/com.microsoft.Excel/Data/Library/Preferences/com.microsoft.Excel.securebookmarks.plist",
        "/Library/Containers/com.microsoft.Powerpoint/Data/Library/Preferences/com.microsoft.Powerpoint.securebookmarks.plist"};
    int length = sizeof(paths)/sizeof(paths[0]);

    switch(choice){
        case 0: rmFile(paths[choice]);
            break;
        case 1: rmFile(paths[choice]);
            break;
        case 2: rmFile(paths[choice]);
            break;
        case 3:
            for(int i=0;i<length;i++){
                rmFile(paths[i]);
            }
            break;
        case 4:
            std::exit(EXIT_SUCCESS); //quit app
    }
}

//print HOME path
void printPath(){
    std::cout << "Home path is located at: " << std::getenv("HOME") << "\n";
}