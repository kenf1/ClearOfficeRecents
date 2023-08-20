#print text in console
print("This script will clear the recent files list for 1 or more MS Office applications on macOS.")
print("Script by KF: https://github.com/kenf1")

string_paths <- c("/Library/Containers/com.microsoft.Word/Data/Library/Preferences/com.microsoft.Word.securebookmarks.plist",
                 "/Library/Containers/com.microsoft.Excel/Data/Library/Preferences/com.microsoft.Excel.securebookmarks.plist",
                 "/Library/Containers/com.microsoft.Powerpoint/Data/Library/Preferences/com.microsoft.Powerpoint.securebookmarks.plist")

#change to root dir
setwd("~/")

#concat root dir w/ string_path
#rm file
lapply(string_paths,function(x){
  full_path <- paste(getwd(),x,sep="")
  file.remove(full_path)
  print("Successfully cleared recents file list.")
})
