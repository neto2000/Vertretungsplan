<script>

  import { admin_rows, changed_rows } from "../stores";

  export let id;


  let row_list;

  admin_rows.subscribe((value) => {

    row_list = value[id];
  })


  let changed = [];

  changed_rows.subscribe((value) => {
    changed = value;
  })


  function row_to_changed_list() {

    for (let i = 0; i < changed.length; i++) {

      if (changed[i].id == row_list.id) {

        return; 
      }
    }

    console.log("changed")
    
    changed_rows.set([...changed, row_list]);
  }


</script>

<tr class="row">

  <td><input type="text" bind:value={row_list.class} on:input={row_to_changed_list} class="input-field" style="width: 90%;"></td>

  <td><input type="number" bind:value={row_list.start_hour} on:input={row_to_changed_list} class="input-field" style="width: 90%;"></td>

  <td><input type="text" bind:value={row_list.old_fach} on:input={row_to_changed_list} class="input-field" style="width: 90%;"></td>

  <td><input type="text" bind:value={row_list.away} on:input={row_to_changed_list} class="input-field" style="width: 90%;"></td>

  <td><input type="text" bind:value={row_list.sub} on:input={row_to_changed_list} class="input-field" style="width: 90%;"></td>

  <td><input type="text" bind:value={row_list.new_fach} on:input={row_to_changed_list} class="input-field" style="width: 90%;"></td>

  <td><input type="text" bind:value={row_list.room} on:input={row_to_changed_list} class="input-field" style="width: 90%;"></td>

  <td><input type="text" bind:value={row_list.typ} on:input={row_to_changed_list} class="input-field" style="width: 90%;"></td>

  <td><input type="text" bind:value={row_list.info} on:input={row_to_changed_list} class="input-field" style="width: 90%;"></td>

</tr>

<style>
  .input-field {

    width: auto;
  }

  ::placeholder {

    color: black;

    opacity: 1;

  }

  

</style>
