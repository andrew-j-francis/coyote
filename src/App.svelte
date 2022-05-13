<script>
    import CreateCharacter from "./CreateCharacter.svelte";
    import CombatScreen from "./CombatScreen.svelte";
    import {Button} from "carbon-components-svelte";
    import "carbon-components-svelte/css/g90.css";

    let showStartScreen = true;
    let showCharacterCreator = false;
    let showCombatScreen = false;
    let character;

    function startNewGame() {
        showStartScreen = false;
        showCharacterCreator = true;
    }

    function startCombat(event) {
        character = event.detail.character;
        showCharacterCreator = false;
        showCombatScreen = true;
    }

</script>

<style>
    .new-game-button {
        display: grid;
        place-items: center;
    }
</style>

{#if showStartScreen}
    <div class="new-game-button">
        <Button on:click={startNewGame}>New Game</Button>
    </div>
{/if}

{#if showCharacterCreator}
    <CreateCharacter on:characterCreate={startCombat}></CreateCharacter>
{/if}

{#if showCombatScreen}
    <CombatScreen character={character}></CombatScreen>

{/if}
