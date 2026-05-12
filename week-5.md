# Overview
Hard deadline lab assignment next Tuesday, An Introduction to CHERI reading ch 1-5 & skim ch 8, read chapter 18 & 19

## Lab Assignment
### Summary

## CHERI Reading 
https://www.cl.cam.ac.uk/techreports/UCAM-CL-TR-941.pdf

### Brief summary
CHERI (Capability Hardware Enhanced RISC Instructions) is meant to be a sidekick to current ISA by extending their capabilites when it comes to memory protection. The key of CHERI is the transition from traditional integer pointers to capabilites, which while pointer like are descriptions of permissions that "refer to data, code, and objects in protected ways." A capability contains a validity tag, bounds, permissions, and object type. These track whether the capability can be used in load/stores/IF, the area of address space the capability is authorized for loads/stores/IF, how restricted the capability is in terms of loads/stores/IF, and whether the capability is sealed thus not being able to be modified or dereferenced. It should be noted that CHERI capabilities are 2 times the size of regular instructions for the given ISA.

A large takeaway from this paper for me was the idea of capability monotonicity, which is the principle that "when any instruction constructs a new capability (except in sealed capability manipulation and exception raising), it cannot exceed the permissions and bounds of the capability from which it was derived." This is, or shouold be, an inherently safe way of contstructing a pointer substitute such that you have very fine control over which areas of memory they may be able to access.

Yet another important point of the CHERI architecture is that it's "modular." As CHERI is meant to be an extension of existing ISA's it can be used across a variety of systems and so called CHERI Aware code is portable across underlying architectures.

### List the major contributions (what is it that's novel) compared to previous work
The entire concept of extending conventional ISAs seems novel to me, but that may be due to my lack of experience with CS research papers. They also seemed dedicated to getting a full working prototype for CHERI software to an extent I've not seen in previous papers, even getting adaptions of Clang and FreeBSD working in their software stack. 

One of the more novel things for me to read about was the Sail and L3 languages for ISA specification and how usefule it seemed to be for documentation and test cases (at the very least). There was also the "machine-checked Isabelle proof" they used to mathematically prove their language was secure, which although briefly mentioned seemed incredibly interesting. 

### Strengths / Weaknesses
The entire paper seemed very strong aside from the fact they never explicitly acknowledged how extreme the overhead was. Seeing as speed and efficiency are the name of the game when it comes to almost all computer systems these days, the lack of explicit reference there was interesting (the word "overhead" only appears 7 times in the paper). I find it hard to believe that a doubling of traditional ISA instructions, permissions checks, implementation of a new tag table, and so on would not lead to a significant system slow-down.

### Questions
"In some architectures, the additional implied ADD to relocate data accesses may present a microarchitectural challenge as it can affect a key critical path." - What path? This claim was made but never expanded on

"We therefore proposed a hierarchical tag table able to exploit the variable density of asserted tags across pages of DRAM" - How does this work? 

There seems to be subsantial overlap between CHERI and Twizzler, is that the case? If so, how has Twizzler solved some of the issues presented at the end of the paper, such as: 
- How can we evaluate the impact of CHERI on security, and also validate that concrete  software artifacts are effective in using CHERI to improve security?
- How should CHERI be used by, and more generally interact with, virtualization environments such as those found in contemporary cloud environments – including operational services such as virtual-machine migration?
- How can the architectural formal models that we have developed (and are continuing  to develop) be used to support verification of both supporting microarchitecture and the software stacks built over CHERI?

Would extending the ISA length mess with thread safety? How do Twizzler threads work?

## Reflection
