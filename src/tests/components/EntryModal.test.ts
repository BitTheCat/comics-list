import { fireEvent, render, waitFor } from '@testing-library/vue';
import { describe, expect, it } from 'vitest';
import type { Entry } from '../../types';
import EntryModal from '../../components/entries/EntryModal.vue';

describe('EntryModal', () => {
  it('opens in create mode', () => {
    const { getByRole } = render(EntryModal, {
      props: {
        modelValue: true,
        series_id: 1,
        entry: null,
      },
    });

    expect(
      getByRole('heading', { name: 'Create entry' }),
    ).toBeTruthy();

    expect(
      getByRole('button', { name: 'Create' }),
    ).toBeTruthy();

    expect(
      getByRole('button', { name: 'Cancel' }),
    ).toBeTruthy();
  });

  it('adds unique tags and removes them', async () => {
    const { getByPlaceholderText, getByRole, queryByText } = render(EntryModal, {
      props: {
        modelValue: true,
        series_id: 1,
        entry: null,
      },
    });

    const input = getByPlaceholderText('Add tags separated by commas');
    await fireEvent.update(input, 'Marvel, Superhero, marvel');
    await fireEvent.click(getByRole('button', { name: 'Add' }));

    expect(queryByText('Marvel')).toBeTruthy();
    expect(queryByText('Superhero')).toBeTruthy();
    await fireEvent.click(getByRole('button', { name: 'Remove tag Marvel' }));
    expect(queryByText('Marvel')).toBeNull();
  });

  it('previews and removes a selected cover', async () => {
    const { container, getByAltText, getByRole, queryByAltText } = render(EntryModal, {
      props: {
        modelValue: true,
        series_id: 1,
        entry: null,
      },
    });

    const input = container.querySelector('#entry-cover') as HTMLInputElement;
    const cover = new File(['cover'], 'cover.png', { type: 'image/png' });
    Object.defineProperty(input, 'files', { value: [cover] });
    await fireEvent(input, new Event('change', { bubbles: true }));

    await waitFor(() => expect(getByAltText('Cover preview')).toBeTruthy());
    await fireEvent.click(getByRole('button', { name: 'Remove cover' }));
    expect(queryByAltText('Cover preview')).toBeNull();
  });

  it('opens in edit mode', () => {
    const entry: Entry = {
      id: '1',
      issue_number: '1',
      title: 'Test',
      series_id: 1,
      pub_year: 2021,
      pub_month: 1,
      cover_path: null,
      tags: [],
    };

    const { getByRole, getByDisplayValue } = render(EntryModal, {
      props: {
        modelValue: true,
        series_id: 1,
        entry,
      },
    });

    expect(
      getByRole('heading', { name: 'Edit entry' }),
    ).toBeTruthy();

    expect(
      getByRole('button', { name: 'Update' }),
    ).toBeTruthy();

    expect(getByDisplayValue('Test')).toBeTruthy();
    expect(getByDisplayValue('2021')).toBeTruthy();
  });
});
