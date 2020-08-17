# Temperature Conversion

A project for learning Rust.

## Project Reference

The Rust Programming Language Chapter 3.5 Summary.

## Technologies

* Rust

## How to use

Input a temperature and a temperature type, using the following format:

```temperature type```

For example:

```38 C```

```-12.8 f```

```0 Kelvin```

The computer will then print the temperature in Celcius, Fahrenheit, and Kelvin.

## Restrictions

Temperature and type must be input precisely as follows:

```temperature type```

The computer cannot cope with extra symbols between temperature and type (including a second space). It also cannot cope with no spaces.

The computer looks only at the first letter input for type. It does not attempt to match to the closest type. For example, if the following is input:

```0 Celvin```

then the computer will interpret the temperature type as Celcius, and will convert the temperature from Celcius.

