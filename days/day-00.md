# Day 00 - Setting up the goals 🎯

As today is the first day and it's Saturday (I have the all day for me 🥳), I'll take a bit of time to explain my objectives and to give some context about that project. It will also be the occasion for me to clarify a bit my ideas and to write things down.

## Challenge goals

​	Let's start with the begining ! Usually, people tend to do that challenge to learn a new language, a new framework or as side notes when following a course. That's not my case here, I already have some knowledge of Machine Learning and I'm already coding in Rust and Python (the languages that I'll use during this challenge). My goal is quite different, it will be more about "forcing" me to book some time every day to work on a topic that I really like ... 🥁... **Embedded Machine Learning**. 

You don't know me but that's already the kind of thing I do at work, so what the hell about doing a #100DaysOfCode challenge on that topic ? 💀. 

The answer is quite simple, I want to keep track of what I'm learning and to share a bit of it at the same time. In addition to that, I'm planing to focus on the deployment of algorithms on really ressourced constrained devices (such as the Arduino nano 33 BLE). That's a topic that I'm not covering yet at work and that doesn't seems to be widely covered on the web (don't tell me about TF-lite it's not in Rust and if every thing goes well, I will probably end up having my own version of if at the end of that challenge 😌).

So here it is, this challenge will be all about **deploying ML workloads on tiny devices using Rust** and to deal with all the challenges related to that. I will try to cover the following topics:

- Run Rust on embedded targets
- Code simple neural networks
- Benchmark code
- Explore exising solutions and how they work
- Optimize models for embedded deployment (quantization, size reduction)
- Doing projects to perform different tasks (gesture recognition, image recognition, ..)



## Week 1 goals

Knowing this challenge will mainly be about endurance, I'll try not to overbook the first weeks even if being super motivated really tempt me to do so. That's why I decided to dedicate this week to do setup. I will mainly focus on having a good developement environment for my experiments. 

I'll will use two different boards in addition to my MacbookPro:

- **Raspberry Pi 3B**: This board will allow me to experiment on an embedded target with an OS, which will be much more easier to start. But don't worry, I'm convinced we will face interesting challenges even on this device.
- **Arduino nano 33 BLE**: This board will be the real thing, no OS, no official support. We will have a lot of fun trust me.

That being said, what are this week goals ? 🤓

- Setting up a Raspberry Pi
- Cross compile Rust code to run on the Raspberry Pi
- Setting up an Arduino nano 33 BLE (in other words, find a way to flash it with Rust code)
- Cross compile Rust code to run on the Arduino
- Cleanup my personal blog in case I write some articles about intersting topics covered during the challenge
