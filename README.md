# Description
Chronofile is a simple fast file searcher that can search the whole disk in around 20s (this time can depend on your disk size, disk speed and cpu capabilities) built in rust (blazingly fast !!)

# Installation
First install ``chronofile.exe`` from [releases](https://github.com/DevNitrate/chronofile/releases/tag/release) and from here there are two options:

**1:**
  -put the file into your system32 and it's done. simply open the command prompt and you're ready to go.
  
**2:**
  -put the file where you want and then create an evironment variable linked to the folder where the file is stored. simply use it through the command prompt again.

  # Use
  the use for chronofile is: ``chronofile \<file name or part of file name\> *c*``. Note: ``-c`` is an optional parameter which, when enabled only searches the current directory and it's sub-directories instead of teh whole C disk.
