import { render } from '@testing-library/vue';
import { describe, expect, it } from 'vitest';
import Table from './Table.vue';

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
});
