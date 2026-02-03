import { useEffect, useState } from 'react';
import { Modal, Form, Input, Transfer, Divider } from 'antd';
import { UserOutlined, PhoneOutlined, MailOutlined } from '@ant-design/icons';
import type { Customer, Arcade } from '../types';
import { useArcades } from '../hooks/useArcades';

interface CustomerModalProps {
  open: boolean;
  mode: 'create' | 'edit';
  customer?: Customer;
  onSubmit: (values: any) => void;
  onCancel: () => void;
  loading?: boolean;
}

interface TransferItem {
  key: string;
  title: string;
  description: string;
  disabled: boolean;
}

export const CustomerModal = ({
  open,
  mode,
  customer,
  onSubmit,
  onCancel,
  loading,
}: CustomerModalProps) => {
  const [form] = Form.useForm();
  const { data: arcades = [] } = useArcades();
  const [targetKeys, setTargetKeys] = useState<string[]>([]);

  // Build transfer data source - show unassigned arcades + arcades assigned to this customer
  const transferDataSource: TransferItem[] = arcades
    .filter((arcade: Arcade) => {
      // Include if: unassigned OR assigned to current customer
      return arcade.customer_id === null || arcade.customer_id === customer?.id;
    })
    .map((arcade: Arcade) => ({
      key: String(arcade.id),
      title: arcade.name,
      description: arcade.machine_id.substring(0, 8) + '...',
      disabled: false,
    }));

  useEffect(() => {
    if (open) {
      if (mode === 'edit' && customer) {
        form.setFieldsValue({
          name: customer.name,
          phone_number: customer.phone_number || '',
          email: customer.email || '',
        });
        setTargetKeys(customer.arcade_ids.map(String));
      } else {
        form.resetFields();
        setTargetKeys([]);
      }
    }
  }, [open, mode, customer, form]);

  const handleOk = async () => {
    try {
      const values = await form.validateFields();
      const submitValues = {
        ...values,
        phone_number: values.phone_number || undefined,
        email: values.email || undefined,
        arcade_ids: targetKeys.map(Number),
      };
      onSubmit(submitValues);
    } catch {
      // Validation failed
    }
  };

  const handleTransferChange = (newTargetKeys: React.Key[]) => {
    setTargetKeys(newTargetKeys as string[]);
  };

  const filterOption = (inputValue: string, option: TransferItem) =>
    option.title.toLowerCase().includes(inputValue.toLowerCase()) ||
    option.description.toLowerCase().includes(inputValue.toLowerCase());

  return (
    <Modal
      title={mode === 'create' ? 'New Customer' : 'Edit Customer'}
      open={open}
      onOk={handleOk}
      onCancel={onCancel}
      confirmLoading={loading}
      destroyOnClose
      width={680}
      okText={mode === 'create' ? 'Create' : 'Save'}
    >
      <Form
        form={form}
        layout="vertical"
        initialValues={{ name: '', phone_number: '', email: '' }}
        style={{ marginTop: 16 }}
      >
        <Form.Item
          name="name"
          label="Customer Name"
          rules={[
            { required: true, message: 'Please enter customer name' },
            { min: 2, message: 'Name must be at least 2 characters' },
          ]}
        >
          <Input
            placeholder="Enter customer name"
            prefix={<UserOutlined style={{ color: '#64748b' }} />}
            size="large"
          />
        </Form.Item>

        <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 16 }}>
          <Form.Item
            name="phone_number"
            label="Phone"
          >
            <Input
              placeholder="Phone number"
              prefix={<PhoneOutlined style={{ color: '#64748b' }} />}
            />
          </Form.Item>

          <Form.Item
            name="email"
            label="Email"
            rules={[{ type: 'email', message: 'Please enter a valid email' }]}
          >
            <Input
              placeholder="Email address"
              prefix={<MailOutlined style={{ color: '#64748b' }} />}
            />
          </Form.Item>
        </div>

        <Divider style={{ margin: '8px 0 16px' }} />

        <div style={{ marginBottom: 8, fontWeight: 500, color: '#e2e8f0' }}>
          Arcade Assignments
        </div>

        <Transfer
          dataSource={transferDataSource}
          titles={['Available', 'Assigned']}
          targetKeys={targetKeys}
          onChange={handleTransferChange}
          render={(item) => item.title}
          filterOption={filterOption}
          showSearch
          listStyle={{
            width: 290,
            height: 280,
          }}
          style={{ justifyContent: 'center' }}
        />
      </Form>
    </Modal>
  );
};
