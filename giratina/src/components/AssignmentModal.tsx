import { useEffect, useState } from 'react';
import { Modal, Form, Select } from 'antd';
import { useArcades } from '../hooks/useArcades';
import { useGames } from '../hooks/useGames';
import { useGameVersions } from '../hooks/useGameVersions';
import type { Assignment } from '../types';

interface AssignmentModalProps {
  open: boolean;
  mode: 'create' | 'edit';
  assignment?: Assignment;
  onSubmit: (values: any) => void;
  onCancel: () => void;
  loading?: boolean;
}

export const AssignmentModal = ({
  open,
  mode,
  assignment,
  onSubmit,
  onCancel,
  loading,
}: AssignmentModalProps) => {
  const [form] = Form.useForm();
  const [selectedGameId, setSelectedGameId] = useState<number | null>(null);

  const { data: arcades = [] } = useArcades();
  const { data: games = [] } = useGames();
  const { data: versions = [] } = useGameVersions(selectedGameId);

  useEffect(() => {
    if (open) {
      if (mode === 'edit' && assignment) {
        form.setFieldsValue({
          arcade_id: assignment.arcade_id,
          game_id: assignment.game_id,
          assigned_version_id: assignment.assigned_version_id,
        });
        setSelectedGameId(assignment.game_id);
      } else {
        form.resetFields();
        setSelectedGameId(null);
      }
    }
  }, [open, mode, assignment, form]);

  const handleOk = async () => {
    try {
      const values = await form.validateFields();
      onSubmit(values);
    } catch {
      // Validation failed
    }
  };

  const handleGameChange = (gameId: number) => {
    setSelectedGameId(gameId);
    // Reset version selection when game changes
    form.setFieldValue('assigned_version_id', undefined);
  };

  return (
    <Modal
      title={mode === 'create' ? 'Create Assignment' : 'Edit Assignment'}
      open={open}
      onOk={handleOk}
      onCancel={onCancel}
      confirmLoading={loading}
      destroyOnHidden
      width={600}
    >
      <Form form={form} layout="vertical">
        <Form.Item
          name="arcade_id"
          label="Arcade"
          rules={[{ required: true, message: 'Please select an arcade' }]}
        >
          <Select
            placeholder="Select an arcade"
            disabled={mode === 'edit'}
            options={arcades.map((arcade) => ({
              label: `${arcade.name} (${arcade.machine_id})`,
              value: arcade.id,
            }))}
            showSearch
            filterOption={(input, option) =>
              (option?.label ?? '').toLowerCase().includes(input.toLowerCase())
            }
          />
        </Form.Item>

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
            onChange={handleGameChange}
            showSearch
            filterOption={(input, option) =>
              (option?.label ?? '').toLowerCase().includes(input.toLowerCase())
            }
          />
        </Form.Item>

        <Form.Item
          name="assigned_version_id"
          label="Version to Assign"
          rules={[{ required: true, message: 'Please select a version' }]}
        >
          <Select
            placeholder="Select a version"
            disabled={!selectedGameId}
            options={versions.map((version) => ({
              label: `${version.version} (${version.gcs_path})`,
              value: version.id,
            }))}
            showSearch
            filterOption={(input, option) =>
              (option?.label ?? '').toLowerCase().includes(input.toLowerCase())
            }
          />
        </Form.Item>
      </Form>
    </Modal>
  );
};
