* world is simulated using rapier
* players are moved using a character controller
* only player deltas are sent over the network
* the system runs in ticks

## What happens in a tick?

### Client

* Receive incoming server state events
  * Check if predicted states match server states
  * Rollback if necessary, and re-simulate to the present using the stored commands
  * Mark the received server state as confirmed, and drop all previous state and command data

* Handle Inputs
  * Accumulate inputs to command
  * Send command to server
  * Add command to history

* Calculate a new predicted state
  * Predict commands from other players
  * Apply commands
  * Push new state to history

### Server

* Receive Player Commands
  * Ignore out-of-date commands
  * Store commands for future ticks
  * Advise Sync Corrections if commands come to slow, or too quickly
  * Push to command buffer

* Calculate a new state
  * For critical commands (e.g. fire), rollback to adjust for the clients ping
  * Perform hit registration
  * Predict commands from other players
  * Apply current commands

* Calculate deltas for each player
  * Occlusion Culling
  * (Frustum Culling)
  * Send deltas to their respective clients
