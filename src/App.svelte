<script>
    import {invoke} from '@tauri-apps/api/tauri'

    let name = "";
    let strength = 0;
    let stamina = 0;

    function createCharacter() {
        invoke('create_character', {
            name: name,
            strength: strength,
            stamina: stamina
        }).then((character) => console.log(character))
    }
</script>

<div class="create-character-page">
    <label for="character_name">Character Name</label>
    <input id="character_name" bind:value={name} type="text">

    <label for="character_strength">Strength: {strength}</label>
    <input id="character_strength" type="range" bind:value={strength} min="0" max="10">

    <label for="character_stamina">Stamina {stamina}</label>
    <input id="character_stamina" type="range" bind:value={stamina} min="0" max="10">

</div>
<button on:click={createCharacter}>
    Create Character
</button>


<style>
    .create-character-page {
        display: flex;
        flex-direction: column;
        flex-wrap: nowrap;
        justify-content: space-around;
    }
</style>