<script>
  
  export let is_edit = false;

  let table = [
    [{"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"}, 
     {"fach":"Deutsch", "teacher":"Hr. Mustermann", "room": "442"}, 
     {"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"},
     {"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"},
     {"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"}],
    [{"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"}, 
     {"fach":"Deutsch", "teacher":"Hr. Mustermann", "room": "442"}, 
     {"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"},
     {"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"},
     {"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"}],
    [{"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"}, 
     {"fach":"Deutsch", "teacher":"Hr. Mustermann", "room": "442"}, 
     {"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"},
     {"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"},
     {"fach":"Mathe", "teacher":"Hr. Mustermann", "room": "442"}],
  ];

</script>


<div>
  <tr class="row">
    <th class="table-head"></th>
    <th class="table-head">Montag</th>
    <th class="table-head">Dienstag</th>
    <th class="table-head">Mittwoch</th>
    <th class="table-head">Donnerstag</th>
    <th class="table-head">Freitag</th>
  </tr>
  {#each table as row, i (i)}
    <tr class="row">
    <td class="table-cell">{i + 1}. Stunde</td>
    {#each row as j}
        <td class="table-cell">
          <b style="font-weight: 600;">{j.fach}</b>
          Hr. Mustermann
          <br>
          440

        </td>
    {/each}

    </tr>
  {/each} 
</div>


<style>
  .row {

    background-color: var(--secondary);

  }

  .table-head {

    background-color: var(--secondary);

    vertical-align: middle; 

    width: 12vw;

  }

  .table-cell {

    vertical-align: center;

    height: 6.5vh;
  }
</style>
