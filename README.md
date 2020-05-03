# Longshoreman

[![codecov](https://codecov.io/gh/danieleades/longshoreman/branch/master/graph/badge.svg)](https://codecov.io/gh/danieleades/longshoreman)
![Continuous integration](https://github.com/danieleades/longshoreman/workflows/Continuous%20integration/badge.svg)


This little Docker client started life as an attempt to drag Shiplift up to date, but it grew arms and legs.

There's not many endpoints implemented just yet, but my aim is that as they're added, they are well-tested and strongly-typed.

it's early days, and i'm happy to accept notes of interest in particular endpoints, as well as pull requests.

## Contributing

- Given the sheer number of endpoint which exist, and the number of options available for each, I'm absolutely depending on pull requests to help me tackle some of these
- the complicated logic of request handling is centralised, simplifying the implementation of the individual endpoints
- i'm unlikely to accept pull requests that do not have unit tests. Later, I would also like comprehensive integration tests for *each* endpoint. (this is tricky initially, because combinations of multiple endpoints will be required in order to create meaningful integration tests)