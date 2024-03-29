# Stream

## Stream Functions
<details>
  <summary>Stream functions</summary>
  
This is pseudo code for the stream functions.
  ```javascript

    fn constructStream (
      streamName, // MyStream
      streamDescription, // This is a stream
      streamLeader, // Martin Maurer
      streamMembers[], // [Martin Maurer, John Doe, Jane Doe]
      streamValue, // 100000
      streamBalance, // 0
      streamDeadline, // 2020-01-01
      streamTags[], // [finance, money, Web3, Ethereum, blockchain, crypto, cryptocurrency]
    ) {
      stream = {
        name: streamName,
        description: streamDescription,
        leader: streamLeader,
        members: streamMembers[],
        value: streamValue,
        balance: streamBalance,
        deadline: streamDeadline,
      }
      return stream
    }

    fn constructDeveloper (
        name: string, // Martin Maurer
        github: string, // empea-careercriminal
        skill: string[], // [javascript, solidity, python, rust, go, c++, c, java, html, css, bash, git]
        hourlyRate: number, // 100
        availability_in_range: number, // 0
    ) {
      developer = {
        name: name,
        github: github,
        skill: skill[],
        hourlyRate: hourlyRate,
        availability_in_range: availability_in_range,
      }
      return developer
    }

    constructStream(streamName,streamDescription,streamLeader,streamMembers[],streamTags)
    bestFit(Developers, Budget, Timeline, Skillset)
    addDeveloper(Developer)
    removeDeveloper(Developer)
    amISiloed(Stream)
    amIPartOf(Program)
  ```
</details>
<br/><br/>

## Stream Questions

### Which dev should I pick for next project under the given requirements x,y and z.
- x: is the budget
- y: is the timeline
- z: is the skillset

1. Map budget per hour to developer's hourly rate
    - closest to budget per hour is the best fit
    - input: budget per hour, developer's hourly rate
2. Map timeline to developer's availability
    - largest timeline to the developer with the most availability wins
    - input: timeline, developer's availability
3. Map skillset(required) to skillset(developer)
    - closest match wins
    - input: skillset(required), skillset(developer)

Generalize the above steps to a function that takes in a list of developers and returns the best fit. If there is a tie, pick the developer with the most availability

<details>
  <summary>Find best fit developer</summary>

This is pseudo code for the bestFit() function.
```js

    fn bestFit(Developers, Budget, Timeline, Skillset) -> Developer {
        mapBudgetPerHourToDeveloperHourlyRate(Developers, Budget)
        mapTimelineToDeveloperAvailability(Developers, Timeline)
        mapSkillsetToDeveloperSkillset(Developers, Skillset)
        return bestFitDeveloper(Developers)
}
  ```
</details>
<br/><br/>

### How does removing a dev from a stream introduce new dependencies.
*Dependencies* are the things that a stream requires to produce the intended output. In this case, the stream is the project and the output is the value.

The dependencies are the developers, the budget, the timeline and the skillset.

When a developer is removed from the stream, the dependencies are recalculated and the stream is re-evaluated.

Removing a dev:
- remove dev with the least availability
- remove dev with the least salary
- remove dev with the least skillset

Calculating new dependencies:
- Siloed knowledge
- Siloed skillset
- Siloed availability

Mitigation Steps:
- Calculate best fit developer
- Add best fit developer to the stream

<!-- If this fails to recover, then we try the next step:
- Extend the timeline

If this fails to recover, then we try the **very last** step:
- Increase the budget -->

### How does adding a dev to a stream introduce new dependencies.
Adding a new dev to the stream introduces new dependencies. The new dependencies are the new dev's skillset, availability and salary.
When this happens, the stream is re-evaluated.

### Am I siloed?
Generate a function that evaluates if stream is siloed. The function takes in a list of developers and returns a boolean. If the stream is siloed, then the function returns true. If the stream is not siloed, then the function returns false.

<details>
  <summary>Am I siloed?</summary>

```javascript
fn amISiloed(Stream) -> boolean {
    let siloedKnowledge = evaluateSiloedKnowledge(Stream)
    let siloedSkillset = evaluateSiloedSkillset(Stream)
    let siloedAvailability = evaluateSiloedAvailability(Stream)
    return siloedKnowledge || siloedSkillset || siloedAvailability
}
fn evaluateSiloedKnowledge(Stream) -> boolean {
    let knowledge = Stream.map(dev => dev.knowledge)
    return knowledge.length === 1
}
fn evaluateSiloedSkillset(Stream) -> boolean {
    let skillset = Stream.map(dev => dev.skillset)
    return skillset.length === 1
}
fn evaluateSiloedAvailability(Stream) -> boolean {
    let availability = Stream.map(dev => dev.availability)
    return availability.length === 1
}
fn isSiloed(Stream) -> boolean {
    return amISiloed(Stream)
}
```
</details>
<br></br>

### How do I recover from siloed knowledge?
- Replace a dev from the the stream
- Inform the program lead of the siloed knowledge (Programs have different functions to deal with this for you)


# Program

## Program Functions:
<details>
  <summary>Program functions</summary>

```javaScript
calculateDependencies(Stream)
sumStreamDependencies(Program)
subtractStreamFromProgram(Stream, Program)
```
</details>
<br></br>

## Program Questions
”How much siloing do I introduce to a program, when I remove stream x from it.”
- siloed vs non-siloed could be calculated by the number of dependencies



<details>
  <summary>Calculate Program Dependencies</summary>

```javascript
fn calculateDependencies(Stream) -> number {
    let dependencies = Stream.map(dev => dev.dependencies)
    return dependencies.length
}

fn sumStreamDependencies(Program) -> number {
    let dependencies = Program.map(stream => calculateDependencies(stream))
    return dependencies.reduce((acc, curr) => acc + curr)
}

fn subtractStreamFromProgram(Stream, Program) -> number {
    let newProgram = Program.filter(stream => stream !== Stream)
    return sumStreamDependencies(newProgram)
}
```
</details>
<br></br>


# Forum
## Forum Functions
*to be added*
## Forum Questions
*to be added*


# Entities

**Stream** is a a vision and a goal to provide some kind of value to a customer, user, or client.

**Core Team** is working on the primary objective to deliver the maximum value to the customer/client. It’s about the “what” and “why”.

**Enabling Team** help the stream on organizational and process-related matters or also on non-functional technical matters. They enable the core team.

**Stream Lead** is accountable for the overall success of the Stream. This corresponds to the role of the Product Owner in the Scrum Framework.

**Contributor** is responsible for the success of the Stream, by contributing in their defined subject or specialty. This corresponds to the role of the Scrum Team Member in the Scrum Framework.

**Program** consist of Several streams operating in the same context or serving similar markets.

**Program Lead** is responsible for driving the program’s overall success. This role can be seen and implemented as a Chief Product Owner type role as defined in the Scrum@Scale framework.

**Impact Forum** align the efforts of multiple streams, e.g. to maximize the value for a customer, group of customers or market.
The Impact Forum usually consists of the stream leads, with one of the stream leads taking the lead → Impact Forum Lead.

**Experience Forum** could also consists of stream leads from different Programs

**Experience Lead** not defined

