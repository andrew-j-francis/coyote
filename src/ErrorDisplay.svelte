<script>
    export let errorMessage = "Test Message";
</script>

<style>
    .error-message {
        display: grid;
        place-items: center;
        color: darkred;
        font-weight: bold;
        font-size: large;
    }
</style>


<div class="error-message">
    {errorMessage}
</div>