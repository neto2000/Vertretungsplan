<script>
    import Header from './lib/Header.svelte'
    import Plan from './lib/Plan.svelte'
    import Personal from './lib/Personal.svelte';
    import Stundenplan from './lib/Stundenplan.svelte';

    const Pages = {
        Main: 'Main',
        Stundenplan: 'Stundenplan',
        About: 'About',
        Anhaenge: 'Anhaenge',
    }


    let current_page = Pages.Main;
</script>

<main>
    <Header pages={Pages} bind:current_page={current_page} />

    {#if current_page == Pages.Main}
        <Personal />
        <Plan />
    {:else if current_page == Pages.Stundenplan}
        <Stundenplan />
    {/if}
</main>

<style>
</style>
