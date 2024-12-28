
# File command from linux for windows

# Guide 


Identify file type:<br>
`neofile identify my_file.txt`<br>
read file:<br>
`neofile cat my_file.txt`<br>
write to file: <br>
`neofile write _`<br>
verify file:<br>
`neofile vf my_file.txt`<br>
append to file:<br>
`neofile apnd my_file.txt` use the `--no-newline` flag to use no newline when appending<br>
copy file:<br>
`neofile cp my_file.txt --dest="my_dest.txt"` use the `--show-bytes` to show how many bytes were copied<br>
move file:<br>
`neofile move my_file.txt --dest="my_dest.txt"`. do NOT put a / at the end of your srcpath or destpath, and make sure your srcfile is in the same directory as your terminal.<br>

changelist:<br>

v0.1.5<br>
added the copy cmd<br>
added move command<br>

v0.1.4<br>
added new flag to apnd command<br>
