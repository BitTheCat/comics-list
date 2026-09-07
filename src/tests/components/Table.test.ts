import { fireEvent, render } from '@testing-library/vue';
import { describe, expect, it } from 'vitest';
import Table from '../../components/Table.vue';

describe('Table', () => {
  it('renders headers and rows from its props', () => {
    const { getByText } = render(Table, {
      props: {
        fields: [{ key: 'title', label: 'Title' }],
        items: [
          { id: 'series-1', title: 'Spawn' },
          { id: 'series-2', title: 'Savage Dragon' },
        ],
      },
    });

    expect(getByText('Title')).toBeTruthy();
    expect(getByText('Spawn')).toBeTruthy();
    expect(getByText('Savage Dragon')).toBeTruthy();
  });

  it('emits the selected row when selection is enabled', async () => {
    const { emitted, getByRole } = render(Table, {
      props: {
        fields: [{ key: 'title', label: 'Title' }],
        items: [{ id: 'series-1', title: 'Spawn' }],
        selectable: true,
        selectedId: 'series-1',
      },
    });

    const row = getByRole('row', { name: 'Spawn' });
    expect(row.getAttribute('aria-selected')).toBe('true');

    await fireEvent.click(row);
    expect(emitted()['row-click']).toEqual([
      [{ id: 'series-1', title: 'Spawn' }],
    ]);
  });
});
