use deno_core::extension;
use deno_core::op2;

#[op2(fast)]
fn op_get_players() -> Result<f64, deno_core::error::AnyError> {
    Ok(100.0)
}

extension!(
    oogabooga,
    ops = [op_get_players],
    esm_entry_point = "ext:oogabooga/src/ext.js",
    esm = [
        "src/ext.js",
        "node_modules/@oogaboogagames/game-core/dist/ext/GameBase.js",
        "node_modules/@oogaboogagames/game-core/dist/ext/Player.js",
        "node_modules/@oogaboogagames/game-core/dist/ext/LobbyStage.js",
    ],
);
