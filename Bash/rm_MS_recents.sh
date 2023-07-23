#!/bin/zsh

#intro text (source: https://www.tech-otaku.com/mac/removing-entries-microsoft-office-apps-recent-files-lists-macos/)
echo "This script will clear the recent files list for 1 or more MS Office applications on macOS."
echo "Script by KF: https://github.com/kenf1"
echo ""

#prompt user: which config files to rm
echo "Which MS Office application do you want to clear the recent files list?"
echo "Options are: Word, Excel, PowerPoint, All"
echo ""

read user_choice

#rm file(s)
if [ ${user_choice} == "Word" ]
    then
        rm ~/Library/Containers/com.microsoft.Word/Data/Library/Preferences/com.microsoft.Word.securebookmarks.plist
        echo "Successfully cleared recents files list for: MS Word."
elif [ ${user_choice} == "Excel" ]
    then
        rm ~/Library/Containers/com.microsoft.Excel/Data/Library/Preferences/com.microsoft.Excel.securebookmarks.plist
        echo "Successfully cleared recents files list for: MS Excel."
elif [ ${user_choice} == "PowerPoint" ]
    then
        rm ~/Library/Containers/com.microsoft.Powerpoint/Data/Library/Preferences/com.microsoft.Powerpoint.securebookmarks.plist
        echo "Successfully cleared recents files list for: MS PowerPoint."
elif [ ${user_choice} == "All" ]
    then
        rm ~/Library/Containers/com.microsoft.Word/Data/Library/Preferences/com.microsoft.Word.securebookmarks.plist ~/Library/Containers/com.microsoft.Excel/Data/Library/Preferences/com.microsoft.Excel.securebookmarks.plist ~/Library/Containers/com.microsoft.Powerpoint/Data/Library/Preferences/com.microsoft.Powerpoint.securebookmarks.plist
        echo "Successfuly cleared recents files list for: MS Word, Excel, and PowerPoint."
else
    echo "The accepted options are: Word, Excel, PowerPoint, or All. Please try again."
fi