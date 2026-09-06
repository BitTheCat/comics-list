import { render } from '@testing-library/vue';
import { describe, expect, it } from 'vitest';
import type { Entry } from '../../types';
import EntryModal from '../../components/EntryModal.vue';

describe('EntryModal', () => {
  it('opens in create mode', () => {
    const { getByRole } = render(EntryModal, {
      props: {
        modelValue: true,
        series_id: '1',
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

  it('opens in edit mode', () => {
    const entry: Entry = {
      id: '1',
      issue_number: '1',
      title: 'Test',
      series_id: '1',
      pub_year: 2021,
      pub_month: 1,
      cover_path: null,
      tags: [],
    };

    const { getByRole, getByDisplayValue } = render(EntryModal, {
      props: {
        modelValue: true,
        series_id: '1',
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
