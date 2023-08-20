print("This script will clear the recent files list for 1 or more MS Office applications on macOS.\n Script by KF: https://github.com/kenf1\n")

string_paths = ["/Library/Containers/com.microsoft.Word/Data/Library/Preferences/com.microsoft.Word.securebookmarks.plist",
                 "/Library/Containers/com.microsoft.Excel/Data/Library/Preferences/com.microsoft.Excel.securebookmarks.plist",
                 "/Library/Containers/com.microsoft.Powerpoint/Data/Library/Preferences/com.microsoft.Powerpoint.securebookmarks.plist"]

import os

os.getcwd()

#remove each file in string_paths using list comprehension + try/except
try:
    [os.remove(item) for item in string_paths]
    print("Successfully cleared recents file list.")
except:
    print("File does not exist")
