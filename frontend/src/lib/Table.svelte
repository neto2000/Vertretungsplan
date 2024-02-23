<script>
  import TableRow from "./TableRow.svelte";

  import { onMount } from 'svelte';

  let test_rows = 15

  let db_rows = [];
  
  onMount(async () => {
    await get_rows();
  });

  async function get_rows() {

    const res = await fetch('/get_rows', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          id: 1,
        }),
    })
    
    db_rows = await res.json()

    console.log(db_rows[0].id);

  
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

    {#each db_rows as row}
      <TableRow row={row} /> 
    {/each}

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
