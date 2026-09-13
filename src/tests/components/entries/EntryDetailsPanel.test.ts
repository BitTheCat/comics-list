import { fireEvent, render } from '@testing-library/vue';
import { describe, expect, it } from 'vitest';
import EntryDetailsPanel from '../../../components/entries/EntryDetailsPanel.vue';

const entry = {
  id: 'entry-1',
  series_id: 1,
  issue_number: '7',
  title: 'The Return',
  pub_year: 2024,
  pub_month: 3,
  cover_path: '/covers/entry-1.jpg',
  tags: ['Hero'],
  condition: 'NM' as const,
};

describe('EntryDetailsPanel', () => {
  it('renders its placeholder without a selected entry', () => {
    const { getByText } = render(EntryDetailsPanel);

    expect(
      getByText('Select an entry from the table to preview its details.'),
    ).toBeTruthy();
  });

  it('renders entry details and emits its actions', async () => {
    const { container, emitted, getByRole, getByText } = render(EntryDetailsPanel, {
      props: {
        entry,
        publication: 'Marzo 2024',
      },
    });

    expect(getByRole('heading', { name: 'The Return' })).toBeTruthy();
    expect(getByText('Marzo 2024')).toBeTruthy();
    expect(getByText('Hero')).toBeTruthy();

    await fireEvent.click(getByRole('img', { name: 'Cover' }));
    await fireEvent.click(getByRole('button', { name: 'Edit' }));

    await fireEvent.click(getByRole('button', { name: 'Delete' }));
    const confirmDeleteButton = Array.from(container.ownerDocument.querySelectorAll('button'))
      .find((button) => button.textContent?.trim() === 'Delete' && button.className.includes('btn-error')) as HTMLButtonElement;
    await fireEvent.click(confirmDeleteButton);

    expect(emitted().view).toEqual([[entry]]);
    expect(emitted().edit).toEqual([[entry]]);
    expect(emitted().delete).toEqual([[entry]]);
  });
});
