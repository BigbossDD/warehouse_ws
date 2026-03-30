from setuptools import find_packages, setup

package_name = 'warehouse_robot'

setup(
    name=package_name,
    version='0.0.0',
    packages=find_packages(exclude=['test']),
    data_files=[
        ('share/ament_index/resource_index/packages',
            ['resource/' + package_name]),
        ('share/' + package_name, ['package.xml']),
    ],
    install_requires=['setuptools'],
    zip_safe=True,
    maintainer='bigboss',
    maintainer_email='bigboss@todo.todo',
    description='TODO: Package description',
    license='TODO: License declaration',
    extras_require={
        'test': [
            'pytest',
        ],
    },
    entry_points={
    'console_scripts': [
        'sensor_node = warehouse_robot.sensor_node:main',
        'task_node = warehouse_robot.task_node:main',
        'delivery_action_server = warehouse_robot.delivery_action_server:main',
        'delivery_action_client = warehouse_robot.delivery_action_client:main',
        'safety_monitor_node = warehouse_robot.safety_monitor_node:main'
    ],
},
)
