---
pagetitle: CV
---

# Nan Wang

I am graduating at Fall 2018, looking for a full time job now.

## Contact

* wangnangg@gmail.com 
* (984)377-9350 
* [github.com/wangnangg](https://www.github.com/wangnangg) 
* [wangnangg.net](http://wangnangg.net)
* 1500 Duke University Road, APT# D3B, Durham, NC 27701

## Education

* 2009 Fall - 2013 Spring **Beihang University (formerly known as Beijing University of Aeronautics and Astronautics)**
  * B.E. in Automation
  * GPA 3.72/4.0
* 2013 Fall - 2016 Spring **Beihang University**
  * M.E in Aeronautical Engineering
  * GPA 3.58/4.0
* 2016 Fall - 2018 Fall (*Expected Graudation date*) **Duke University**
  * M.S. in  Electrical and Computer Engineering
  * GPA 3.97/4.0 

## Work Experience

* 2017 Summer (May - Aug.) **Samsung Semiconductor Inc**.
  * Enterprise SSD Firmware Engineer (Intern)

## Skill

* Proficient in C & C++. 
* Proficient in Python. 
* Familiar with Linux.
* Familiar with C#.
* Dabbled in some other languages.
  * SML
  * Julia

## Project

### Ground Object Tracking Based-on AR.Drone (2013)
* Language: **C**

### UAV Route Planning Verification (2014)
* Language: **C & C#**

### Fault-Tolerant Flight Controller Simulation  (2014)

* Language: **C & C#**

### Linux Kernel Aging Detection (2015)

* Language: **C**

### UnitFL (2015)

* Language: **C# & C++**
* Description: [UnitFL](https://marketplace.visualstudio.com/items?itemName=Wangnangg.UnitFL) is a fault localization extension for Visual studio. It can dynamically instrument C# code to record line coverage during unit testing. Then, coverage information and corresponding results (passed or failed) of unit tests are used to locate bugs. More info about this tool can be found [here](https://marketplace.visualstudio.com/items?itemName=Wangnangg.UnitFL).

### Toy CPU on FPGA (2016)

* Language: **VHDL**
* Description: This is a course project of [ECE 550 - Fundamentals of Computer Systems and Engineering](http://people.duke.edu/~tkb13/courses/ece550-2016fa/). We implemented an un-pipelined single-cycle CPU that has a small instruction set (much like a subset of MIPS). More detailed requirement can be found [here](docs/ece550_hw4.pdf). Later in the course, we also implemented a cache for this CPU.

### Tiger Language Compiler (2017)

* Language: **SML**
* Description: This is a semester-long project of [ECE 553 - Compiler Construction](https://adhilton.pratt.duke.edu/ece-553-compiler-construction). We followed the book [Modern Compiler Implementation in ML](https://www.cs.princeton.edu/~appel/modern/ml/) and used a whole semester
  to implement a complete Tiger compiler. Tiger language a small language that is
  designed by the author of the book. Its specification can be found [here](https://cs.nyu.edu/courses/fall13/CSCI-GA.2130-001/tiger-spec.pdf). The project was divided into multiple
  smaller projects reflecting different phases of a compiler (lexing, parsing,
  type checking, etc.). The source code of my implementation is on [github](https://github.com/wangnangg/tiger-compiler). 

### Stochastic Petri Net Solver (2017)

* Language: **C++**
* Description: Stochastic Petri Net (SPN) is a modeling language widely used in reliability, availability and performability modeling. I implemented a numerical solver that can covert a stochastic Petri Net into the equivalent Markov chain and solve this Markov chain for probabilities and cumulative times at steady state as well as at a certain time point. I also implemented a discrete event simulator that can simulate a SPN model and output measures of interest, along with their confidence intervals. The source code of this solve can be found [here](https://github.com/wangnangg/sanity).

### Buffer overflow attack (2018)

* Tool: **GDB, Ropper**
* Description: This is a course project of [COMPSCI 590 - Computer Security](). Our purpose was to get a root shell by exploiting a root process with buffer overflow vulnerability. We used [return-oriented programming](https://en.wikipedia.org/wiki/Return-oriented_programming) to achieve this goal. A detailed report of this project is [here](https://docs.google.com/document/d/1rB6mzIEsocCYIaVvCTJnW53PE4LAyPTqYqzu7Y1TBQw/edit?usp=sharing).

### Root kit (2018)

* Language: **C**
* Description: This is a course project of [Systems Programming & Engineering](http://people.duke.edu/~tkb13/courses/ece650-2018sp/). I implemented a Kernel module that intercept system calls to perform various malicious attacks, one of which is to hide the trace of an executable of my choice, including the file and its running process. This is done by censoring the contents of read and readdir system calls.

### Network File System with Caching (2018)

* Language: **C++**
* Description: This is a semester-long project of [Enterprise Storage
  Architecure](http://people.duke.edu/~tkb13/courses/ece590-stor/). The file
  system is implemented in user space with FUSE library. We have a server
  program that can serve a directory from a remote host. The client (linked with
  FUSE) can then mount a file system on a local directory. The remote directory
  can be accessed as if it exists locally (just like Samba or CIFS). The client
  has a caching subsystem that caches file attributes and contents, which
  greatly improves sequential read/write performance of our file system.

## Course

| Term | Course | Topic | Grade|
|----------|-----------|---------|:----:|
|2016 Fall|ECE 550D | [Fundamentals of Computer Systems and Engineering](http://people.duke.edu/~tkb13/courses/ece550-2016fa/) |	A+|
|2016 Fall|	ECE 555  |	Probability for Electrical and Computer Engineers |	A |
|2016 Fall | MATH 561  |	Numerical Linear Algebra | 	A+|
|2017 Sprng | ECE 553 |	[Compiler Construction](https://adhilton.pratt.duke.edu/ece-553-compiler-construction) | A |
|2017 Sprng | ECE 590 - 12 | Discrete Event Simulation                                    | A |
|2017 Fall | STA 621 |	Applied Stochastic Processes | A- |
|2018 Sprng | COMPSCI 520 |	Numerical Analysis | A |
|2018 Sprng | COMPSCI 590 |	[Computer Security](https://www2.cs.duke.edu/courses/spring18/compsci590.1/) | A |
|2018 Sprng | ECE 650 |	[Systems Programming & Engineering](http://people.duke.edu/~tkb13/courses/ece650-2018sp/) | A+ |
|2018 Fall | ECE 551D | [Programming, Data Structures, and Algorithms in C++](http://adhilton.pratt.duke.edu/ece-551) | A+ |
|2018 Fall | ECE 590 - 02 | [Enterprise Storage Architecure](http://people.duke.edu/~tkb13/courses/ece590-stor/) | A |


## Publication

1. Nan Wang, Zheng Zheng, Zhenyu Zhang and Cheng Chen, "FLAVS: A Fault Localization Add-In for Visual Studio," *2015 IEEE/ACM 1st International Workshop on Complex Faults and Failures in Large Software Systems (COUFLESS)*, 2015.
2. Cheng Chen, Nan Wang, "UnitFL: A fault localization tool integrated with unit test", *Computer Science and Network Technology (ICCSNT) 2016 5th International Conference on*, 2016.
3. Zheng Zheng, Kishor S. Trivedi, Nan Wang and Kiu Qiu, "Markov Regenerative Models of WebServers for Their User-Perceived Availability and Bottlenecks", *IEEE Transactions on Dependable and Secure Computing*, 2018.
4. Harish Sukhwani, Nan Wang, Kishor Trivedi and Andy Rindos, "Performance Modeling of Hyperledger Fabric (Permissioned Blockchain Network)", in *The 17th IEEE International Symposium on Network Computing and Applications*, 2018.
