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

  // 1 - Alice can create game
  const create_game_result = await alice.callSync('main', 'create_game', {
    opponent: bob.agentId,
    timestamp: 0
  })
  console.log(create_game_result)
  t.equal(create_game_result.Ok.length, 46)

  // 2 - Bob can make first move, creating token
  const move_1_result = await bob.callSync('main', 'make_move', {
    new_move: {
      game: create_game_result.Ok,
      move_type: { CreateToken: { distance: 4 } },
      timestamp: 1
    }
  })
  console.log(move_1_result)
  t.equal(move_1_result.Err, undefined)

  // 3 - Alice then can't move because it's Bob's turn due to rosette
  const move_2_result = await alice.callSync('main', 'make_move', {
    new_move: {
      game: create_game_result.Ok,
      move_type: { CreateToken: { distance: 3 } },
      timestamp: 2
    }
  })
  console.log(move_2_result)
  t.equal(move_2_result.Ok, undefined)

  // 4 - Bob moves again
  const move_3_result = await bob.callSync('main', 'make_move', {
    new_move: {
      game: create_game_result.Ok,
      move_type: { CreateToken: { distance: 3 } },
      timestamp: 3
    }
  })
  console.log(move_3_result)
  t.equal(move_3_result.Err, undefined)

  // 5 - Now Alice can move
  const move_4_result = await alice.callSync('main', 'make_move', {
    new_move: {
      game: create_game_result.Ok,
      move_type: { CreateToken: { distance: 2 } },
      timestamp: 4
    }
  })
  console.log(move_4_result)
  t.equal(move_4_result.Err, undefined)

})


diorama.run()
