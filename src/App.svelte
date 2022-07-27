<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import type { Notification } from './lib/models/notifications';

  const notificationsPromise = invoke<Notification[]>('get_notifications', {
    codeHost: 'GitHub',
    opts: { all: true },
  });
</script>

<main>
  {#await notificationsPromise}
    <p>Loading...</p>
  {:then notifications}
    {#each notifications as notification}
      <p>{notification.id}</p>
    {/each}
  {:catch err}
    <p>{err}</p>
  {/await}
</main>

<style>
</style>
