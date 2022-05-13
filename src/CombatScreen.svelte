<script>
    import {invoke} from "@tauri-apps/api/tauri";
    import {Button} from "carbon-components-svelte";

    export let character;
    let enemy;
    let showEnemy = false;


    function createEnemy() {
        invoke('create_enemy').then((newEnemy) => {
            console.log(newEnemy);

            enemy = newEnemy;
            showEnemy = true;
        })

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
            Name: {character.name}
        </div>
        <div>
            Gold: {character.gold}
        </div>
        <div>
            Strength: {character.strength}
        </div>
        <div>
            Stamina: {character.stamina}
        </div>
    </div>
    <div class="entity-data">
        {#if !showEnemy}
            <Button on:click={createEnemy}>Spawn Enemy</Button>
        {/if}

        {#if showEnemy}
            <div>
                Name: {enemy.name}
            </div>
            <div>
                Gold: {enemy.gold}
            </div>
            <div>
                Strength: {enemy.strength}
            </div>
            <div>
                Stamina: {enemy.stamina}
            </div>
        {/if}
    </div>
</div>

<hr>
<div style="font-size: x-large; padding: 1rem;">
    Combat Log
</div>
<hr>

