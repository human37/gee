# gee
repository manager and automation tool written in rust

#### problem
The problem I have is managing where I clone repositories I find on github. I hate having to remember the directory I cloned something, especially when I am done checking it out and I have to remember to delete it from its specific directory also. I also like checking out lots of git repositories, and managing storage on my computer can become an issue after some time.

#### solution
My solution is my package manager called “gee”. It will allow you to clone any git repository by saying “gee install {url}”. It will then install it into a globally accessible directory, and it will allow me to access them from any directory by saying “gee open {name}”. I will also be able to run “gee list” and it will show me all of my current installed git repositories. It will also solve my storage issue by having a queue, where it will only store my 5 most recent git repository clones, by automatically deleting the oldest one when I download a new one. I can also change this number from 5 to any arbitrary number I would like. I also would like to make commands such as “gee update”, and it will run a git pull command in all of my git repositories. Depending on how long these features take me, I will also look into adding more advanced features, such as searching for a Makefile and building automatically, etc.
