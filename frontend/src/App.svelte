<script>
    import Header from './lib/Header.svelte'
    import Plan from './lib/Plan.svelte'
    import Personal from './lib/Personal.svelte';
    import Stundenplan from './lib/Stundenplan.svelte';


    let main_page = false;
</script>

<main>
    <Header />

    {#if main_page}
        <Personal />
        <Plan />
    {:else}
        <Stundenplan />
    {/if}
</main>

<style>
</style>
