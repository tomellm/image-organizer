backup="working_files/backup"
work="working_files/original"
dump="working_files/dump"

rm -r "working_files/dump/*"
rm -r $work 2> /dev/null
mkdir $work
cp $backup/* $work/ 