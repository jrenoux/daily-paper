# Daily Paper
Given a list of papers (pdf)  and a list of org-roam notes, opens a random paper without an associated org-roam note. 

## How to use
 - Change the papers and note folders in 'config/config.json'
 - Check the regular expression in main.rs, line 19. My papers and notes names are formatted 'authornameYYYYword' (e.g. renoux2020unified) so I filter to check that the papers follow this format to avoid temporary files or other files. 
 - Launch the executable and read your daily paper!