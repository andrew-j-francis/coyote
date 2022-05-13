<script>
    import {Button} from "carbon-components-svelte";

    export let character;
    let enemy;


    function createEnemy() {

    }
</script>

<style>
    .entity-data-section {
        display: grid;
        grid-template-columns: 50% 50%;
        margin: 1rem;
    }

    .entity-data {
        display: flex;
        flex-direction: column;
        flex-wrap: nowrap;
        border-color: gray !important;
        border-style: solid !important;
        border-width: thick;
        padding: 1rem;
        font-size: large;
    }

</style>

<div class="entity-data-section">
    <div class="entity-data">
        <div>
            {character.name}
        </div>
        <div>
            {character.gold}
        </div>
        <div>
            {character.strength}
        </div>
        <div>
            {character.stamina}
        </div>
    </div>
    <div class="entity-data">
        <Button on:click={createEnemy}></Button>
    </div>
</div>

<hr>
<div style="font-size: x-large; padding: 1rem;">
    Combat Log
</div>
<hr>

