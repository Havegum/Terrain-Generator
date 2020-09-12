<script>
import { getContext, setContext, afterUpdate } from 'svelte';
import { Layer } from 'konva/lib/Layer';

const { stage } = getContext('konva');

// If needed, make this a store instead
const layer = new Layer({ listening: false });
$stage.add(layer);

afterUpdate(() => layer.draw());
setContext('layer', layer);
</script>


<slot />
