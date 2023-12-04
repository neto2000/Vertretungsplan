<script>
  import TableRow from "./TableRow.svelte";

  let test_rows = 15
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

    {#each Array(test_rows) as _, i}
      <TableRow /> 
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
