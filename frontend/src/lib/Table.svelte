<script>
  import TableRow from "./TableRow.svelte";

  import { onMount } from 'svelte';


  export let day_id;

  let day_exists = false;

  let db_rows = [];
  
  onMount(async () => {
    await get_rows(day_id);
  });

  $: get_rows(day_id)

  async function get_rows(id) {

    const res = await fetch('/get_rows', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          id: id,
        }),
    })

    console.log(res.status)

    if (res.status != 200) {

      day_exists = false

      return
    }
    
    db_rows = await res.json()

    console.log(db_rows[0].id);

    day_exists = true

  
  }
</script>


<div class="table-container">
  <table class="table">
    <tr>
      <th>Klasse</th>
      <th>Stunde</th>
      <th>altes Fach</th>
      <th>abwesend</th>
      <th>Vertreter</th>
      <th>neues Fach</th>
      <th>Raum</th>
      <th>Typ</th>
      <th>Info</th>
    </tr>


    {#if day_exists}
      
      {#each db_rows as row}
        <TableRow row={row} /> 
      {/each}

    {:else}
      {#each {length: 9} as _, i}
        <td>-</td> 
      {/each}
    {/if}

    

  </table>

</div>


<style>
  .table {
    border-collapse:separate;

    border-spacing: 0px;

    border-radius: 10px;

    border: solid;

    border-width: 1px;

    width: 100%;


    overflow: hidden;


    box-shadow: 0px 4px 4px 2px rgba(0, 0, 0, 0.2);
    
  } 
  .table-container {

    width: 90%;


    margin-top: 15vh;

    margin-bottom: 4vh;



  }
</style>
