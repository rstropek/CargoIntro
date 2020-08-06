# Cargo Introduction

![Deploy slides to Docker Hub](https://github.com/rstropek/CargoIntro/workflows/Deploy%20slides%20to%20Docker%20Hub/badge.svg)

## Project Content

This repository contains slides and samples for my *Cargo Introduction* session. It was originally created for a talk at the [Rust Linz meetup](https://www.meetup.com/de-DE/Rust-Linz/events/271857182/) on Aug. 6th 2020.

## Slides

The slides are based on Markdown and [revealjs](https://revealjs.com/). To simplify handling, this project uses [this revealjs CLI](https://github.com/vivaxy/node-reveal).

The slides are [available online](https://cargo-intro-slides.azurewebsites.net/). If you want to run the slides locally, perform the following steps:

* Clone this repository
* `npm install`
* `npm start`

You can also package the slides in a Docker image. A [Dockerfile](Dockerfile) is part of this repository. If you prefer a ready-made image, simply pull [*rstropek/cargo-intro-slides*](https://hub.docker.com/repository/docker/rstropek/cargo-intro-slides) and run it. Slides are available on port 80 (for details see [*Dockerfile*](Dockerfile)).

## Samples

You find the samples for the session in the [*samples*](samples) folder. Each samples contains a hands-on lab readme file that you can use to practice various Rust and Cargo topics. The slides contain links to the hands-on labs.
