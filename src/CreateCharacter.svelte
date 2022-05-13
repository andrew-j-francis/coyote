<script>
    import {invoke} from "@tauri-apps/api/tauri";
    import {Button, Form, FormGroup, NumberInput, TextInput} from "carbon-components-svelte";
    import ErrorDisplay from "./ErrorDisplay.svelte";
    import {createEventDispatcher} from "svelte";

    const dispatch = createEventDispatcher();

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
            }).then((character) => {
                dispatch('characterCreate', {
                    character: character
                })
            })
        }
    }

</script>

<style>
    .create-character-button {
        display: grid;
        place-items: center;
    }

</style>

{#if (hasError)}
    <ErrorDisplay errorMessage={errorMessage}></ErrorDisplay>
{/if}
<div style="padding: 2rem">
    <Form>
        <FormGroup>
            <TextInput labelText="Character Name" bind:value={name}></TextInput>
        </FormGroup>
        <FormGroup legendText="Stats">
            <NumberInput label="Attribute Points Remaining" value={remainingAttributePoints} readonly></NumberInput>
            <NumberInput label="Strength" min="0" max="10" bind:value={strength}></NumberInput>
            <NumberInput label="Stamina" min="0" max="10" bind:value={stamina}></NumberInput>
        </FormGroup>
    </Form>
</div>

<div class="create-character-button">
    <Button on:click={createCharacter}>
        Create Character
    </Button>
</div>
