<script>
import { getContext, setContext } from 'svelte';
import { Group } from 'konva/lib/Group';

const layer = getContext('layer');

const group = new Group();
layer.add(group);

setContext('group', group);
</script>

<slot/>
