import { useEffect } from 'react';
import { Modal, Form, Input, Select, DatePicker } from 'antd';
import dayjs from 'dayjs';
import { useGames } from '../hooks/useGames';
import type { GameVersion } from '../types';

interface GameVersionModalProps {
  open: boolean;
  mode: 'create' | 'edit';
  version?: GameVersion;
  onSubmit: (values: any) => void;
  onCancel: () => void;
  loading?: boolean;
}

export const GameVersionModal = ({
  open,
  mode,
  version,
  onSubmit,
  onCancel,
  loading,
}: GameVersionModalProps) => {
  const [form] = Form.useForm();
  const { data: games = [] } = useGames();

  useEffect(() => {
    if (open) {
      if (mode === 'edit' && version) {
        form.setFieldsValue({
          game_id: version.game_id,
          version: version.version,
          gcs_path: version.gcs_path,
          release_date: dayjs(version.release_date),
        });
      } else {
        form.resetFields();
      }
    }
  }, [open, mode, version, form]);

  const handleOk = async () => {
    try {
      const values = await form.validateFields();
      // Convert date to ISO string
      const payload = {
        ...values,
        release_date: values.release_date.toISOString(),
      };
      onSubmit(payload);
    } catch (error) {
      // Validation failed
    }
  };

  return (
    <Modal
      title={mode === 'create' ? 'Create Game Version' : 'Edit Game Version'}
      open={open}
      onOk={handleOk}
      onCancel={onCancel}
      confirmLoading={loading}
      destroyOnHidden
      width={600}
    >
      <Form
        form={form}
        layout="vertical"
        initialValues={{ release_date: dayjs() }}
      >
        <Form.Item
          name="game_id"
          label="Game"
          rules={[{ required: true, message: 'Please select a game' }]}
        >
          <Select
            placeholder="Select a game"
            disabled={mode === 'edit'}
            options={games.map((game) => ({
              label: game.name,
              value: game.id,
            }))}
            showSearch
            filterOption={(input, option) =>
              (option?.label ?? '').toLowerCase().includes(input.toLowerCase())
            }
          />
        </Form.Item>

        <Form.Item
          name="version"
          label="Version"
          rules={[
            { required: true, message: 'Please enter version' },
            {
              pattern: /^\d+\.\d+\.\d+$/,
              message: 'Version must be in format X.Y.Z (e.g., 1.0.0)',
            },
          ]}
        >
          <Input placeholder="e.g., 1.0.0" />
        </Form.Item>

        <Form.Item
          name="gcs_path"
          label="GCS Path"
          rules={[{ required: true, message: 'Please enter GCS path' }]}
          extra="Path to the game version folder in GCS"
        >
          <Input placeholder="e.g., Games/MyGame/1.0.0" />
        </Form.Item>

        <Form.Item
          name="release_date"
          label="Release Date"
          rules={[{ required: true, message: 'Please select release date' }]}
        >
          <DatePicker
            showTime
            style={{ width: '100%' }}
            format="YYYY-MM-DD HH:mm:ss"
          />
        </Form.Item>
      </Form>
    </Modal>
  );
};
