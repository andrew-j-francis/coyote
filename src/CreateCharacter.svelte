<script>
    import {invoke} from "@tauri-apps/api/tauri";
    import {Button, NumberInput, TextInput} from "carbon-components-svelte";
    import ErrorDisplay from "./ErrorDisplay.svelte";

    let name = "";
    let strength = 0;
    let stamina = 0;
    let maxAttributePoints = 10;
    let errorMessage = "";
    let hasError = false;
    $: remainingAttributePoints = maxAttributePoints - (strength + stamina);

    function createCharacter() {
        hasError = false;

        if (remainingAttributePoints < 0) {
            hasError = true;
            errorMessage = "Too many points have been used";
        } else {
            invoke('create_character', {
                name: name,
                strength: strength,
                stamina: stamina
            }).then((character) => console.log(character))
        }
    }

</script>

<style>
    .create-character-page {
        display: flex;
        flex-direction: column;
        flex-wrap: nowrap;
        justify-content: space-around;
    }

    .create-character-button {
        display: grid;
        place-items: center;
    }
</style>

{#if (hasError)}
    <ErrorDisplay errorMessage={errorMessage}></ErrorDisplay>
{/if}

<div class="create-character-page">
    <TextInput labelText="Character Name" bind:value={name}></TextInput>
    {remainingAttributePoints}

    <NumberInput label="Strength" min="0" max="10" bind:value={strength}></NumberInput>
    <NumberInput label="Stamina" min="0" max="10" bind:value={stamina}></NumberInput>

</div>

<div class="create-character-button">
    <Button on:click={createCharacter}>
        Create Character
    </Button>
</div>
