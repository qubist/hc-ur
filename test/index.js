const path = require('path')
const tape = require('tape')

const { Diorama, tapeExecutor, backwardCompatibilityMiddleware } = require('@holochain/diorama')

process.on('unhandledRejection', error => {
  // Will print "unhandledRejection err is not defined"
  console.error('got unhandledRejection:', error);
});

const dnaPath = path.join(__dirname, "../dist/hc-ur.dna.json")
const dna = Diorama.dna(dnaPath, 'hc-ur')

const diorama = new Diorama({
  instances: {
    alice: dna,
    bob: dna,
  },
  bridges: [],
  debugLog: false,
  executor: tapeExecutor(require('tape')),
  middleware: backwardCompatibilityMiddleware,
})


// Your tests here

diorama.registerScenario("Can create a new game", async (s, t, {alice, bob}) => {

  const create_game_result = await alice.call('main', 'create_game', {
    opponent: bob.agentId,
    timestamp: 0
  })

  console.log(create_game_result)

  t.equal(create_game_result.Ok.length, 46)

})


diorama.run()
