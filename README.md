# gee
CLI repository manager and automation tool written in rust.

### Overview
If you like temporarily cloning repositories, then Gee is the tool for you! It allows you to clone any repository on github by running:

    $ gee clone <url>

Gee will then download the repository into ```~/.gee/tmp/```. Gee clones them into this directory, in order to implement a temporary repository queue. It allows the user to have a configurable maximum number of repositories they would like Gee to store. And when the user clones enough repositories on the queue to go over that limit, Gee will automatically remove the oldest repository in order to pop it off of the queue. You can run the command:

    $ gee list

in order to see all of your repositories you have currently installed using gee, as well as it's index in the repository queue. In order to open a repository you have cloned with gee, you can run the command:

    $ gee open <index>

Gee will then create a symbolic link of the desired repository in your current working directory. When you decide you are done using that repository, you can run the command:

    $ gee done

in order to remove the symbolic link, and close the opened repository. You can also run the command:

    $ gee keep <index> [path]

in order to copy a repository out of Gee's queue, if you decide you would like to keep it. If you don't specify the path parameter, it will assume the user's currently working directory. You can also run the command:

    $ gee help

in order to recieve more information about the functionality of Gee. 

### Configuration
By default, Gee will assume the maximum number of repositories to store as 5. You can configure this by adding a ```.geerc``` file in your home directory. And for example, if you would like to change the maximum number to 9, add this line in your ```.geerc``` file:
```
queue_size 9
```

### Installation
This is available for you macintosh user's as a [homebrew](https://brew.sh) tap. In order to install run:

    $ brew tap human37/gee
      brew install gee

You can also complile from source and install using cargo. First clone the repository, and then within the repository run:

    $ cargo install --path .
