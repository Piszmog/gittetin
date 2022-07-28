<script lang="ts">
  import type { Notification } from '../models/notifications';
  import Item from './Item.svelte';

  export let notifications: Notification[];
</script>

<div class="bg-white rounded-lg border border-gray-200 w-full text-gray-900">
  {#each notifications as notification}
    <Item {notification} />
  {/each}
</div>
