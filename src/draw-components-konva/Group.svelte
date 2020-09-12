<script>
import { getContext, setContext, beforeUpdate } from 'svelte';
import { Group } from 'konva/lib/Group';

const layer = getContext('layer');
const parentGroup = getContext('group');
const target = parentGroup || layer;

export let zIndex = 1;

const group = new Group();
target.add(group);

beforeUpdate(() => group.zIndex(zIndex));

setContext('group', group);
</script>

<slot/>
