<script lang="ts">
  import type { Notification } from '../models/notifications';

  export let notification: Notification;
</script>

<button
  class="text-left
  px-6
  py-2
  border-b border-gray-200
  w-full
  hover:bg-gray-100 hover:text-gray-500
  focus:outline-none focus:ring-0 focus:bg-blue-600 focus:text-white
  transition
  duration-100
  cursor-pointer"
>
  {notification.title}
</button>
